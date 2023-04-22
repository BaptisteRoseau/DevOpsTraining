# Managing GitLab Workflow

## Groups

I created a `DevOps` group in GitLab to be able to manage multiple repositories for this project if necessary, and manage labels, CI/CD pipelines and runners at the group level.

To move the `DevOps Training` project into this new group it was as easy as going to `DevOps Training` -> `Settings` -> `General` -> `Advanced` -> `Transfer Project` and transfer it under the `DevOps` group.

As this changed the URL, I had to update my `.git/config`'s URLs to replace `Shynamo` into `devops`:

```toml
[core]
	repositoryformatversion = 0
	filemode = true
	bare = false
	logallrefupdates = true
[remote "origin"]
	url = ssh://git@localhost:2022/devops/devops-training.git
	fetch = +refs/heads/*:refs/remotes/origin/*
[branch "main"]
	remote = origin
	merge = refs/heads/main
[lfs]
	repositoryformatversion = 0
	url = "http://localhost:2080/devops/devops-training.git/info/lfs"
	pushurl = "http://localhost:2080/devops/devops-training.git/info/lfs"
[url "http://localhost:2080/"]
	insteadOf = http://localhost/
[lfs "http://localhost:2080/Shynamo/devops/devops-training.git/info/lfs"]
	access = basic
[lfs "http://localhost:2080/devops/devops-training.git/info/lfs"]
	access = basic
```

### Group Labels

I created the following labels in the group namespace:

<p align="center">
  <img src="assets/gitlab_labels.png" />
</p>

My workflow will be as follows:

There are four **stages** of a task:

- Whenever a task comes to mind, I will create a [GitLab Issue](https://docs.gitlab.com/ee/user/project/issues/):
  - Issues have the label `Draft` until the are described enough to be sent into `TODO`
  - `TODO` issues are ready to develop and could be implemented by anyone knowing the project thanks to their description
  - `DOING` are all started issues, even if they are paused
  - `DONE` is for issues that have been resolved and merged

`Draft` allows me to quickly create an issue having just a clear title, without describing it yet and break my focus on the current task.

`Draft` and `DONE` could be considered as duplicates of `Open` and `Closed`. However, `DONE` could be used as a verification by team leads before actually closing the issue in an actual development team.

There are three **kinds** of issues:

- `Feature`: A new feature to implement, or task to perform to improve the codebase
- `Bug`: A bug found in the application, to be fixed
- `Report`: A report to write like this one while/after performing a task

Each issue must have at lease a **kind** and a **stage** (`Draft` or `TODO`) when created. [Merge Request](https://docs.gitlab.com/ee/user/project/merge_requests/#merge-requests) should only inherit the **kind** of their parent issue, if any.

Also, I want to automate the label update from `TODO` to `DOING` when a merge request a created from an issue, and from `DOING` to `DONE` when all the merge requests of an issue have been merged.

I my project I will also close the Issues with the former, but in a team the `DONE` tasks should be reviewed by a team leader to know what is going on and to make sure he/she sees it before closing the issue.

#### Issue Templates

First, to make sure labels are setup automatically I use [issue template](https://docs.gitlab.com/ee/user/project/description_templates.html#create-an-issue-template) as well as [quick actions](https://docs.gitlab.com/ee/user/project/quick_actions.html#gitlab-quick-actions).

However, issue template are not yet supported by GitLab, even it their team [is working on it](https://gitlab.com/gitlab-org/gitlab/-/issues/7749). So I must make the templates in my `DevOps Training` project.

To do so, I created [issue templates](../.gitlab/issue_templates) in [.gitlab/issue_templates](.gitlab/issue_templates) and [merge request templates](../.gitlab/merge_request_templates) in [.gitlab/merge_request_templates](.gitlab/merge_request_templates) in this repository.

It order to [set a template by default](https://docs.gitlab.com/ee/user/project/description_templates.html#set-a-default-template-for-merge-requests-and-issues), I tried using a symbolic link to `Default.md`:

```cmd
baptiste:~/Projects/GitLab/devops/devops-training/.gitlab/merge_request_templates$ ln -s template.md Default.md ln -s template.md Default.md
```

Spoiler: it does not work.

<p align="center">
  <img src="assets/gitlab_template_fail.png" />
</p>


Only the content of the file is used, which is the path written is the symbolic link.

So, instead of a symbolic link I simply rename `template.md` into `Default.md`, and then it works just fine.

<p align="center">
  <img src="assets/gitlab_template_success.png" />
</p>

Unfortunately, the `/unlabel ~Draft ~TODO ~DOING ~DONE` in the merge request template does not work as it seems like issue metadatas are copied after the description's quick actions.

#### Label Update On Merge Requests

TODO


## CI/CD Pipelines

### License Checker

### Security Flaw Checker

