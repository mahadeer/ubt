# Unified Build Tool

> **Please refer to [Docs](https://mahadeer.github.io/ubs/) here.**

> *Documentation for v0.1*

## Getting Started
UBT is a command-line tool used to run pre-defined tasks. These tasks can multiple dependent tasks which will also be executed. UBS tool requires a valid XML configuration in UBS required format. 

UBT is written in [Rust](https://www.rust-lang.org/), so it gives us more advantage on the operating system. One source code will be used across all different operating system, Rust will handle the internal while you compile the source code. UBS will not override any permission set by the operating system, it is preferred for security reasons.

UBT has a number of built-in tasks which will can be used in the build file to simply all the tasks. Please refer to the Tutorial or Reference to explore all the availbel build-in tasks. UBS can be used in any number of ways depending on the project requirement. There are few limitations known as we develop, but will be removed in the future releases.

## When to use?
The main known usage of UBS can be,
 - Automation Build
 - Production Deployment
 - Execute commands
 - Folder Management

*Note: It is a tool, so much more based on your imagination can be acheived*

## How to install?
We are not there yet. Please clone the repo and run from build. Make sure Rust is installed on the local system.
```
$ git clone  https://github.com/mahadeer/ubt.git
$ cd ubt
$ cargo run 'C:/somewhere/build.xml'
```
## Contributing
Please refer [Contribute](https://mahadeer.github.io/ubt/contribute) for details on our code of conduct, and the process for submitting pull requests to us.

## Versioning
We use SemVer for versioning. Version released can be found on the release tags.

## Authors
[Mahadeer Mohamed](https://mahadeer.github.io/)

## License
This project is licensed under the MIT License - see the [Usage](https://mahadeer.github.io/ubt/usage-policy) file for details
