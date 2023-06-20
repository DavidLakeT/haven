use crate::discord::command::commits_command::COMMITS_COMMAND;
use crate::discord::command::pullrequests_command::PULLREQUESTS_COMMAND;
use crate::discord::command::repository_command::REPOSITORY_COMMAND;
use crate::discord::command::status_command::STATUS_COMMAND;
use serenity::framework::standard::macros::group;

#[group]
#[commands(repository, commits, pullrequests, status)]
struct General;