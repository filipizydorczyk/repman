pub fn create_repository_file(path: &str) -> String {
    let result = format!(
        r#"#!/bin/bash

REPOSITORY_PATH="{}"
            
updateCheck() {{
    cd $REPOSITORY_PATH
    git fetch && git --no-pager log --oneline ..origin/master
}}
            
updatePull() {{
    cd $REPOSITORY_PATH
    git pull origin master
}}
            
updateBuild() {{
    echo "Not implemented"
}}
            
buildInstall() {{
    echo "Not implemented"
}}
"#,
        path
    );

    return result;
}
