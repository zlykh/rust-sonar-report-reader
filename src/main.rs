mod enums;

use std::{io};
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{Read};
use clap::{App, Arg, ArgMatches};
use prost::bytes::{Buf};
use prost::{DecodeError, Message};
use crate::enums::{ParsedComponent, ParsedReport};
use crate::report_parts::{ActiveRule, Component, Duplication, Issue, LineCoverage};
use crate::report_parts::line_coverage::{HasCoveredConditions, HasHits};

pub mod report_parts {
    include!(concat!("proto/report_parts.rs"));
}

fn main() {
    main_report_info()
}

#[allow(dead_code)]
fn main_report_info() {
    let args = prepare_cmd_args();
    let report_file = args.value_of("report file")
        .expect("Report file is required");

    let report = read_zip_files(report_file);
    print_results(report_file, report)
}

fn prepare_cmd_args() -> ArgMatches {
    let app = App::new("Sonar Report Reader")
        .version("1.0")
        .about("Displays scanner report in a human readable form")
        .author("Dmitry Zlykh");

    let report_file_arg = Arg::new("report file")
        .long("f")
        .takes_value(true)
        .help("zip report file")
        .default_value("go1s3.zip")
        .required(false);

    let app = app.arg(report_file_arg);

    return app.get_matches();
}

fn read_zip_files(report_file: &str) -> ParsedReport {//-> io::Result<()>{
    let fn_issues_parse = |a: &mut &[u8]| { Issue::decode_length_delimited(a) };
    let fn_coverage_parse = |a: &mut &[u8]| { LineCoverage::decode_length_delimited(a) };
    let fn_duplication_parse = |a: &mut &[u8]| { Duplication::decode_length_delimited(a) };
    let fn_rules_parse = |a: &mut &[u8]| { ActiveRule::decode_length_delimited(a) };
    let fn_component_parse = |a: &mut &[u8]| { Component::decode(a) };

    let mut components: BTreeMap<String, Component> = BTreeMap::new();
    let mut issues: HashMap<String, Vec<Issue>> = HashMap::new();
    let mut coverages: HashMap<String, Vec<LineCoverage>> = HashMap::new();
    let mut duplications: HashMap<String, Vec<Duplication>> = HashMap::new();
    let mut rules_count: HashMap<String, i32> = HashMap::new();

    let file = File::open(String::from(report_file)).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file_bytes: Vec<u8> = Vec::new();
        let mut file = archive.by_index(i).unwrap();
        file.read_to_end(&mut file_bytes).unwrap();

        let path = if let Some(path) = file.enclosed_name() { path } else { continue; };
        let fname = path.file_stem().unwrap().to_str().unwrap();

        if fname.starts_with("component") {
            if let Ok(mut one_elem_list) = parse_proto_file(&mut file_bytes, fn_component_parse) {
                components.insert(fname[10..].to_string(), one_elem_list.remove(0));
            }
        }

        if fname.starts_with("issues") {
            if let Ok(list) = parse_proto_file(&mut file_bytes, fn_issues_parse) {
                issues.insert(fname[7..].to_string(), list);
            }
        }

        if fname.starts_with("coverages") {
            if let Ok(list) = parse_proto_file(&mut file_bytes, fn_coverage_parse) {
                coverages.insert(fname[10..].to_string(), list);
            }
        }

        if fname.starts_with("duplications") {
            if let Ok(list) = parse_proto_file(&mut file_bytes, fn_duplication_parse) {
                duplications.insert(fname[13..].to_string(), list);
            }
        }

        if fname.starts_with("activerules") {
            if let Ok(list) = parse_proto_file(&mut file_bytes, fn_rules_parse) {
                rules_count = list.into_iter()
                    .map(|r| r.rule_repository)
                    .fold(HashMap::new(), |mut acc, c| {
                        *acc.entry(c).or_insert(0) += 1;
                        acc
                    });
            }
        }
    }

    let parsed_components = components.into_iter()
        .map(|(num, comp)|
        ParsedComponent::new(
            comp,
            issues.remove(num.as_str()),
            coverages.remove(num.as_str()),
            duplications.remove(num.as_str()),
        ))
        .collect();

    return ParsedReport::new(rules_count, parsed_components);
}

fn parse_proto_file<T>(buf: &mut Vec<u8>, read_delim_fn: fn(&mut &[u8]) -> Result<T, DecodeError>) -> Result<Vec<T>, io::Error> {
    let mut result = Vec::new();

    // let iss_bytes = fs::read(path)?;
    let mut slice = buf.as_slice();
    while slice.has_remaining() {
        let parsed = read_delim_fn(&mut slice)?;
        result.push(parsed)
    }

    Ok(result)
}

fn print_results(report_file: &str, report: ParsedReport) {
    println!("--- Report: {} ---", report_file);
    println!("Total components: {}", report.components.len());
    println!("Active rules: {:?}", report.rules);

    for pc in report.components {
        let is_test = pc.component.is_test;
        if pc.component.r#ref == 1 && pc.component.project_relative_path.is_empty() {
            println!("\nComponent {} is root: {}", pc.component.r#ref, pc.component.key);
            continue;
        } else {
            println!("\nComponent {}: {}", pc.component.r#ref, pc.component.project_relative_path, );
        }

        println!("Issues: {}", pc.issues.as_ref().map_or(0, |vec| vec.len()));

        //println!("Issue list: {:#?}", pc.issues);


        match pc.duplications {
            None => println!("Duplications: -"),
            Some(dups) => {
                match dups.len() {
                    0 => println!("Duplications: -"),
                    _ => println!("Duplications: {} in {} places", dups.len(), dups.iter()
                        .flat_map(|d| d.duplicate.iter()).count())
                }
            }
        }

        match pc.coverages {
            None => if !is_test { println!("Coverage: -") } else {},
            Some(cov) => {
                let mut exec_lines = 0;
                let mut cov_lines = 0;
                let mut total_cond = 0;
                let mut cov_cond = 0;
                for c in cov {
                    exec_lines += 1;
                    if let Some(hits) = c.has_hits {
                        match hits { HasHits::Hits(hits) => { if hits { cov_lines += 1 } } }
                    }
                    total_cond += c.conditions;
                    if let Some(has_cov) = c.has_covered_conditions {
                        match has_cov {
                            HasCoveredConditions::CoveredConditions(cc) => { cov_cond += cc; }
                        }
                    }
                }
                println!("Coverage: {}/{} lines, {}/{} conditions", exec_lines, cov_lines, total_cond, cov_cond)
            }
        }
    }
}

