# GitHub Issue Closer

This is a simple tool to close all issues in a GitHub repo. This is useful for archiving repos.

GitHub gives the [recommendation below](https://docs.github.com/en/repositories/archiving-a-github-repository/archiving-repositories):
> We recommend that you close all issues and pull requests, as well as update the README file and description, before you archive a repository.

There is also [a discussion](https://github.com/orgs/community/discussions/22554) on the topic here.

## Setup
1. Create a [GitHub app](https://docs.github.com/en/developers/apps/building-github-apps/creating-a-github-app).
2. Grant the app read and write permission for issues and pull requests.
3. Grant access to repos.
4. Install the app. You may have to make the app public if you are working on an org repo.
5. Generate a private key. Save the .pem file.


## Usage
```
github-issue-closer close-all --owner owner --repo repo
```

This program reads from environment variables. In most shells, they can be set inline like so:
```
GITHUB_APP_ID=123456 GITHUB_APP_PRIVATE_KEY_PATH=/path/to/app-name.YYYY-MM-DD.private-key.pem github-issue-closer close-all --owner owner --repo repo
```

## Example Output
```
  1 - PR    419 - Bump node-sass from 4.13.1 to 7.0.0
  2 - Issue 418 - Cannot read property indexOf in version 1.5.3
  3 - Issue 417 - Problem with shared protobuf messages 
  4 - Issue 416 - If you want to support more operations in tabs, such as deleting all, deleting the left, deleting the right, you can refer to the browser tab
  5 - Issue 414 - would be nice if the app removes whitespaces from the path. 
  6 - PR    413 - add instructions for opening after installing with Homebrew
  7 - Issue 411 - Error constructing the request e[this.protoInfo.methodName] is not a function
  8 - Issue 410 - Rebuild with Newer Root Certificates - Unable to Connect to All Addresses Error
  9 - Issue 409 - Importing protos using google.protobuf.Api fails starting version 1.5.3
 10 - Issue 406 - Feature Request: Make the left side panel resizeable and/or auto-resize to fit content
 11 - Issue 404 - [bug]RESOURCE_EXHAUSTED: Received message larger than max (14245240 vs. 4194304)
 12 - Issue 403 - [FeatureRequest] Custom header on Request
 13 - Issue 399 - Error while importing protos : illegal value '['
 14 - Issue 398 - Looking for maintainers
 15 - PR    397 - Bump validator from 13.6.0 to 13.7.0 in /app
 16 - PR    396 - Bump ua-parser-js from 0.7.19 to 0.7.31
 17 - Issue 394 - Broken on Fedora 35
 18 - Issue 390 - Feature Request - Unix domain socket/IPC socket
 19 - Issue 389 - gRPC: Not able to parse protoc-gen-validate defintions
 20 - Issue 387 - Update electron, because of Let's Encrypt DST Root CA X3 certificate expiration 
 ```