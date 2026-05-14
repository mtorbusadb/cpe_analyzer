use crate::model::{BaselineRepoVersion, PrismeBaseline};
use serde::Deserialize;

const BASELINE_LOCK: &str = include_str!("../docs/offline-analyzer/prisme-baseline.lock");

#[derive(Debug, Deserialize)]
struct BaselineLock {
    prisme_backend: RepoLock,
    prisme_ui: RepoLock,
    tss: RepoLock,
}

#[derive(Debug, Deserialize)]
struct RepoLock {
    commit_sha: String,
}

pub fn load_embedded_baseline() -> PrismeBaseline {
    match serde_json::from_str::<BaselineLock>(BASELINE_LOCK) {
        Ok(lock) => PrismeBaseline {
            prisme_backend: BaselineRepoVersion {
                commit_sha: lock.prisme_backend.commit_sha,
            },
            prisme_ui: BaselineRepoVersion {
                commit_sha: lock.prisme_ui.commit_sha,
            },
            tss: BaselineRepoVersion {
                commit_sha: lock.tss.commit_sha,
            },
        },
        Err(_) => fallback_baseline(),
    }
}

fn fallback_baseline() -> PrismeBaseline {
    let unknown = || BaselineRepoVersion {
        commit_sha: "unknown".to_string(),
    };
    PrismeBaseline {
        prisme_backend: unknown(),
        prisme_ui: unknown(),
        tss: unknown(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_baseline_is_loadable() {
        let baseline = load_embedded_baseline();
        assert!(!baseline.prisme_backend.commit_sha.trim().is_empty());
        assert!(!baseline.prisme_ui.commit_sha.trim().is_empty());
        assert!(!baseline.tss.commit_sha.trim().is_empty());
    }
}
