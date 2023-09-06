# Rust in Serverless Applications: A ReInvent Talk Example

## Introduction
This repository serves as a companion to our talk at AWS ReInvent. The talk, titled "Enhancing AWS Lambda with Rust", explores how to leverage the performance and security advantages of Rust in serverless AWS Lambda functions. For more details and to watch the talk, visit [ReInvent Talk](https://hub.reinvent.awsevents.com/attendee-portal/catalog/?search=com306).

## Background Story
### The Journey
- As a company focused on developing AWS management tools, we started with a Minimum Viable Product (MVP) for S3 bucket management.
- Initially, the client was open only to our most loyal customers, and we received overwhelmingly positive feedback.
- Eventually, we opened the client to all of our users.
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
- Multiple Lambda functions are connected to the API Gateway. These functions interact with services like S3, DynamoDB, and Secrets Manager.
- We have multiple runtimes, mostly Python, but also .Net and Java.

## Ways to Integrate Rust into Your Serverless Workload
There are primarily three ways to integrate Rust:

### 1. Rust Bindings
- Use Pyo3 with maturin to create a `.whl` package for use in your Python Lambda.
- Example code can be found under `<path for the github code>`.
- **Benefits**: Speed of development and no need to rewrite the Lambda function.
  
#### Performance Metrics
![Performance Metrics](https://github.com/fun-with-serverless/rustifying-serverless/assets/110536677/b2762a29-6c3f-4044-a345-d35e25ead185)
![Performance Metrics](https://github.com/fun-with-serverless/rustifying-serverless/assets/110536677/21798964-f063-4be3-8da4-6daa0e3270c4)
<img width="588" alt="CleanShot 2023-09-06 at 17 30 42@2x" src="https://github.com/fun-with-serverless/rustifying-serverless/assets/110536677/a9f5035a-6436-443e-aaca-66866678e1e5">


The performance metrics were gathered using SAR-measure-cold-start and aws-lambda-power-tuning.
#### SAR-measure-cold-start
SAR-measure-cold-start is a tool used to measure the cold start latency of AWS Lambda functions. Cold starts occur when an invocation of a function happens without a "warm" instance available, often leading to higher latency. The tool deploys a Serverless Application Repository (SAR) app that mimics real-world scenarios to evaluate the cold start performance of your Lambda function. This helps you understand how long it takes for your function to become responsive in a cold start situation, allowing you to optimize accordingly.

#### AWS Lambda Power Tuning
AWS Lambda Power Tuning is an open-source tool designed to help you optimize the performance characteristics of AWS Lambda functions in terms of cost and speed. It achieves this by running your function at different memory sizes, recording the results, and producing a visualization. The tool allows you to select the optimal configuration that either minimizes cost, maximizes speed, or balances the two based on your specific requirements.

Both of these tools are invaluable for getting the most performance out of your AWS Lambda functions.

### 2. Rewrite the Lambda
- Use `cargo-lambda` and AWS SAM to deploy a full-fledged Rust Lambda.
- Example code can be found under `<path for the github code>`.

#### Performance Metrics
![Performance Metrics](path/to/performance-image-for-rewrite.png)

### 3. Use Extensions
- Additional text on how to use Rust extensions in Lambda functions.

## Building the Example
- This example comes with a devcontainer for both Rust and Python development.
- We use `poetry` for build management. To build and deploy, run:
  ```bash
  poetry install
  poe build-and-deploy
  ```
- The main application resides in s3-admin-app and Rust bindings are in s3-ops-rust-lib.
