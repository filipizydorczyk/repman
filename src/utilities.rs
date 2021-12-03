use crate::commands::{
    BUILD_COMMAND_NAME, CHECK_COMMAND_NAME, INSTALL_COMMAND_NAME, PULL_COMMAND_NAME,
};

pub fn create_repository_file(path: String) -> String {
    let result = format!(
        r#"#!/bin/bash

REPOSITORY_PATH="{}"

{}() {{
    cd $REPOSITORY_PATH
    git fetch && git --no-pager log --oneline ..origin/master
}}

{}() {{
    cd $REPOSITORY_PATH
    git pull origin master
}}

{}() {{
    echo "Not implemented"
}}

{}() {{
    echo "Not implemented"
}}
"#,
        path, CHECK_COMMAND_NAME, PULL_COMMAND_NAME, BUILD_COMMAND_NAME, INSTALL_COMMAND_NAME
    );

    return result;
}
