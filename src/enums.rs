use std::collections::HashMap;
use crate::report_parts::{Component, Duplication, Issue, LineCoverage};

pub struct ParsedReport {
    pub rules: HashMap<String, i32>,
    pub components: Vec<ParsedComponent>,
}

impl ParsedReport {
    pub fn new(rules: HashMap<String, i32>, components: Vec<ParsedComponent>) -> Self {
        Self { rules, components }
    }
}

pub struct ParsedComponent {
    pub component: Component,
    pub issues: Option<Vec<Issue>>,
    pub coverages: Option<Vec<LineCoverage>>,
    pub duplications: Option<Vec<Duplication>>,
}

impl ParsedComponent {
    pub fn new(component: Component, issues: Option<Vec<Issue>>, coverages: Option<Vec<LineCoverage>>,
               duplications: Option<Vec<Duplication>>) -> Self {
        Self { component, issues, coverages, duplications }
    }
}