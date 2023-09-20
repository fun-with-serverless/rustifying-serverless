:construction: Work in Progress: This repository is currently under active development. Features may be incomplete, and documentation may not be up-to-date. Please proceed with caution and feel free to contribute! :construction:



# Rust in Serverless Applications: A ReInvent Talk Example

## Introduction
This repository serves as a companion to our talk at AWS ReInvent. The talk, titled "“Rustifying” serverless: Boost AWS Lambda performance with Rust", explores how to leverage the performance and security advantages of Rust in serverless AWS Lambda functions. For more details and to watch the talk, visit [ReInvent Talk](https://hub.reinvent.awsevents.com/attendee-portal/catalog/?search=com306).

## Background Story
### The Journey
- As a company focused on developing AWS management tools, we started with a Minimum Viable Product (MVP) for S3 bucket management.
- Initially, the product was open only to our most loyal customers, and we received overwhelmingly positive feedback.
- Eventually, we opened the product to all of our users.
- At its peak, the service catered to thousands of clients and tens of thousands of users.
  
### Challenges
- We started experiencing performance bottlenecks and increased IT spending.
- Major architectural changes like adding cache and improving algorithms were implemented.

### The Decision
- We reached a point where our existing runtime, Python, was not sufficient for our performance needs.
- Thus, we decided to leverage Rust for its performance advantages.

This repository illustrates the journey that led us to this point.

## Architecture
![Architecture Diagram](https://github.com/fun-with-serverless/rustifying-serverless/assets/110536677/7492fcfc-2da3-4f09-b163-92f4262e1c2f)
- The architecture consists of an API Gateway with a Lambda authorizer.
- Multiple Lambda functions are connected to the API Gateway. These functions interact with services like S3 and DynamoDB.
- We have multiple runtimes, mostly Python, but also NodeJS.

## Ways to Integrate Rust into Your Serverless Workload
There are primarily three ways to integrate Rust:

### 1. Rust Bindings
- Use Pyo3 with maturin to create a `.whl` package for use in your Python Lambda.
- Example code can be found under `<path for the github code>`.
- **Benefits**: Speed of development and no need to rewrite the Lambda function.

### 2. Rewrite the Lambda
- Use `cargo-lambda` and AWS SAM to deploy a full-fledged Rust Lambda.
- Example code can be found under `<path for the github code>`.
- **Benefits**: Reduced cold starts, thanks to the Rust runtime.

### 3. Use Extensions
- Use extensions to address cross-cutting concerns, such as analytics reporting, using a fast language like Rust.
- Example code can be found under `<path for the github code>`.
- **Benefits**: Reduce Lambda latency by offloading tasks to an external process.

## Building the Example
- This example comes with a devcontainer for both Rust and Python development.
- In case you are not using DevContainers, make sure you have the following prerequisites:
  - Python 3.11
  - Rust - latest version
  - Poetry
  - Poe 
  - AWS SAM
  - NodeJS 18
  - AWS CLI
- We use `poetry` for build management. To build and deploy, run:
  ```bash
  # Install dev tools used for rust compilation.
  poetry install --only=rust-dev-tools
  # Build the rust package
  poe build-lib
  # Build the application and deploy it.
  poe build-and-deploy
  ```
- The main application resides in `s3-admin-app`,  Rust bindings are in `s3-ops-rust-lib` and the extension in `analytics-extension`.

## Local development
 ```bash
  # Install dev tools used for rust compilation.
  poetry install --only=rust-dev-tools
  # Build the rust package
  poe build-lib
  # Add the local rust package
  poetry add .rust-lib/s3_ops_rust-0.1.0-cp311-cp311-manylinux_2_17_x86_64.manylinux2014_x86_64.whl --group dev 
  poetry install
  ```