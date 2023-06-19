use octocrab::{
    models::{pulls::PullRequest, repos::RepoCommit, Repository},
    Octocrab,
};
use serenity::{
    async_trait,
    model::prelude::Ready,
    prelude::{Context, EventHandler, TypeMapKey},
};

#[derive(Clone)]
pub struct Handler {
    pub octocrab: Octocrab,
}

impl TypeMapKey for Handler {
    type Value = Handler;
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }
}

impl Handler {
    pub async fn get_repository_info(
        &self,
        repository_author: String,
        repository_name: String,
    ) -> Result<Repository, octocrab::Error> {
        self.octocrab
            .repos(repository_author, repository_name)
            .get()
            .await
    }

    pub async fn get_repository_commits(
        &self,
        repository_author: String,
        repository_name: String,
    ) -> Result<Vec<RepoCommit>, octocrab::Error> {
        self.octocrab
            .repos(repository_author, repository_name)
            .list_commits()
            .send()
            .await
            .map(|response| response.items)
    }

    pub async fn get_repository_pull_requests(
        &self,
        repository_author: String,
        repository_name: String,
    ) -> Result<Vec<PullRequest>, octocrab::Error> {
        self.octocrab
            .pulls(repository_author, repository_name)
            .list()
            .state(octocrab::params::State::All)
            .send()
            .await
            .map(|response| response.items)
    }
}
