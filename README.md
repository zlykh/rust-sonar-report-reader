# Sonar Report Reader in Rust
Reads SonarQube generated report file and outputs brief details.

## Build
```
PROTOC=./protoc cargo build --release
```

## Run
```
./target/release/rust-sonar-report-reader --f go1s3.zip
```
Output:
```
--- Report: go1s3.zip ---
Total components: 7
Active rules: {"kotlin": 31, "scala": 30, "go": 38, "SQLCC": 20, "grvy": 24, "java": 422, "typescript": 151, "css": 19, "csharpsquid": 36, "python": 134, "flex": 47, "ruby": 30, "xml": 12, "yaml": 25, "Web": 29, "php": 129}

Component 1 is root: GoProject:::go-repo-1:::feature

Component 13: stringutil/reverse.go
Issues: 1
Duplications: 1 in 1 places
Coverage: 0/0 lines, 0/0 conditions

Component 14: stringutil/reverse2.go
Issues: 1
Duplications: 1 in 1 places
Coverage: 4/0 lines, 0/0 conditions

Component 15: stringutil/reverse_test.go
Issues: 1
Duplications: -
Coverage: 4/0 lines, 0/0 conditions

Component 3: animals.go
Issues: 1
Duplications: 1 in 1 places
Coverage: 4/0 lines, 0/0 conditions

Component 6: main.go
Issues: 2
Duplications: 2 in 2 places
Coverage: 17/0 lines, 0/0 conditions

Component 7: other_file.go
Issues: 4
Duplications: -
Coverage: 9/0 lines, 0/0 conditions
```
