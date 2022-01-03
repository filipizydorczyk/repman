use crate::commands::{
    BUILD_COMMAND_NAME, CHECK_COMMAND_NAME, CHECK_COUNT_COMMAND_NAME, INSTALL_COMMAND_NAME,
    PULL_COMMAND_NAME,
};

pub fn create_repository_file(path: String) -> String {
    let result = format!(
        r#"#!/bin/bash

REPOSITORY_PATH="{path}"

{check}() {{
    cd $REPOSITORY_PATH
    git fetch && git --no-pager log --oneline ..origin/master
}}

{pull}() {{
    cd $REPOSITORY_PATH
    git pull origin master
}}

{build}() {{
    echo "Not implemented"
}}

{install}() {{
    echo "Not implemented"
}}

{count}() {{
    {check} | wc -l
}}
"#,
        path = path,
        check = CHECK_COMMAND_NAME,
        pull = PULL_COMMAND_NAME,
        build = BUILD_COMMAND_NAME,
        install = INSTALL_COMMAND_NAME,
        count = CHECK_COUNT_COMMAND_NAME
    );

    return result;
}
