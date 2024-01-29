import { message, danger } from "danger";

const modifiedMd = danger.git.modified_files.join("- ");
message("Changed Files in this PR: \n - " + modifiedMd);
