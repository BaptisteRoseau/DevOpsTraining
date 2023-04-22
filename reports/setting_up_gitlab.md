# Setting Up GitLab local server

I did not want to install a bare metal GitLab server on my machine because I wanted to be able to move, remove or update it freely and easily, so I installed it in a container.

I chose [Podman](https://podman.io/): a rootless container technology. Fortunately a [GitLab container](https://docs.gitlab.com/ee/install/docker.html) exists !

Because I want to enforce rootless setup, I followed GitLab's documentation and created a directory used for persistent GitLab files.

## Install GitLab

```
mkdir GitLabServer
export GITLAB_HOME=$HOME/GitLabServer
```

I installed GitLab using the `podman` CLI:

```bash
podman run --detach \
  --hostname localhost \
  --publish 443:443 --publish 80:80 --publish 22:22 \
  --name gitlab \
  --restart always \
  --volume $GITLAB_HOME/config:/etc/gitlab \
  --volume $GITLAB_HOME/logs:/var/log/gitlab \
  --volume $GITLAB_HOME/data:/var/opt/gitlab \
  --shm-size 256m \
  gitlab/gitlab-ee:latest
```

I ran into the following error.

    Error: statfs /home/baptiste/GitLabServer/logs: no such file or directory

Apparently the `logs`, `data` and `config` directories need to exist in order for the container to be able to use them, so I created the required directories:

```bash
mkdir $GITLAB_HOME/config
mkdir $GITLAB_HOME/logs
mkdir $GITLAB_HOME/data
```

Another error, because I run in rootless mode I am not allowed to use ports < 1024.
I remapped the ports forwarding by prefixing with 20:

```bash
podman run --detach \
  --hostname localhost \
  --publish 20443:443 --publish 2080:80 --publish 2022:22 \
  --name gitlab \
  --restart always \
  --volume $GITLAB_HOME/config:/etc/gitlab \
  --volume $GITLAB_HOME/logs:/var/log/gitlab \
  --volume $GITLAB_HOME/data:/var/opt/gitlab \
  --shm-size 256m \
  gitlab/gitlab-ee:latest
```

And boom ! GitLab was up and running ! I didn't know containers were so easy to manipulate and use to install software !

However, this port mapping caused many issues for cloning repositories and for LFS.
For production environment, I would recommend using an [NGINX](https://www.nginx.com/) with port redirection based on your server's domain in a `docker-compose` pod.
This may be implemented in a future report.

## Configuration

I could connect to GitLab on the url `http://localhost:2080/users/sign_in`.
To sign in, I first needed to connect as `root` using this command to retrieve its password:

    podman exec -it gitlab grep 'Password:' /etc/gitlab/initial_root_password

<p align="center">
  <img src="assets/gitlab_login.png" />
</p>

Then, I could play with GitLab admin panel. First I deactivated sign ups, then created my Shynamo user and finally changed the root's password and kept it in my password manager.

<p align="center">
  <img src="assets/gitlab_admin_page.png" />
</p>

After spending some time exploring GitLab's administrator options and possibilities, I tried to login as Shynamo. An email should have been sent to setup my password but it was not !

At first I tried to setup SMTP, but this was not mandatory so I instead used [this documentation](https://docs.gitlab.com/ee/user/profile/account/create_accounts.html#create-users-in-admin-area) to enforce a default password.

<p align="center">
  <img src="assets/gitlab_password.png" />
</p>


Then, tried to initialize my repository by mirroring the one already create on [GitHub](https://github.com/Shynamo/DevOpsTraining). This was an easy step as GitLab already provide a [GitHub repository importation tool](https://docs.gitlab.com/ee/user/project/import/github.html), only using a *Personal Access Token* for authentication. I generated an access token valid for a day just to retrieve a few repositories.

<p align="center">
  <img src="assets/gitlab_password.png" />
</p>

I just needed to retrieve my SSH key using `cat ~/.ssh/id_rsa.pub` then [upload it to GitLab](https://docs.gitlab.com/ee/user/ssh.html) and I was ready to go.

### Mirror to GitHub

Why did I choose GitLab over GitHub to work ? Because it is widely used in IT companies with private instances, and I prefer its workflow regarding DevOps and project management. Also, I am more used to its GUI than GitHub and wanted to try to setup my own instance.

GitLab provides a GUI to mirror repositories, however things are not as easy as they seem to mirror to GitHub.

<p align="center">
  <img src="assets/gitlab_mirror.png" />
</p>

#### SSH Address

You cannot copy as-is the git URL, and need to modify it from this one

    git@github.com:username/repository.git

To this one

    ssh://git:username@github.com/username/repository.git

#### SSH Public key

To allow my local GitLab server to push to my GitHub repository, I need to add its SSH key to my GitHub account because they key used to mirror repositories is the one of GitLab, not yours. Of course, you should not send you private key !

Once I [found my server's SSH Key](https://docs.gitlab.com/ee/user/project/repository/mirror/#get-your-ssh-public-key) then [added it to GitHub](https://docs.github.com/en/authentication/connecting-to-github-with-ssh/adding-a-new-ssh-key-to-your-github-account#adding-a-new-ssh-key-to-your-account), I could successfully mirror my local repository !

<p align="center">
  <img src="assets/gitlab_mirror_success.png" />
</p>

Now I can fully work on my local GitLab instance and my public GitHub account will be up-to-date without any action required.

I needed to update my `~/.ssh/config` to interact with my local GitLab:

    # GitLab.com
    Host localhost
    PreferredAuthentications publickey
    IdentityFile ~/.ssh/id_rsa.pub

Next I did setup my SSH key on my local GitLab account to ease repositories usage. I want to clone the repositories using SSH instead of HTTP because this is more secure, and I will not have to type any password when interacting with remote repositories.

### Git LFS troubleshooting

I used [Git LFS](https://docs.gitlab.com/ee/topics/git/lfs/) to store report assets more efficiently. It can be easily setup using the following commands:

    $ git clone ssh://git@localhost:2022/Shynamo/DevOpsTraining.git
    Cloning into 'DevOpsTraining'...
    remote: Enumerating objects: 12, done.
    remote: Total 12 (delta 0), reused 0 (delta 0), pack-reused 12
    Receiving objects: 100% (12/12), done.
    Resolving deltas: 100% (1/1), done.
    $ sudo apt-get install -y git-lfs
    $ git lfs install
    Updated git hooks.
    Git LFS initialized
    $ git lfs track "*.png"
    Tracking "*.png"

But when trying to push, I encountered the following error:

    batch response: Post "http://localhost/Shynamo/DevOpsTraining.git/info/lfs/objects/batch": dial tcp 127.0.0.1:80: connect: connection refused
    Uploading LFS objects:   0% (0/4), 0 B | 0 B/s, done.
    error: failed to push some refs to 'ssh://localhost:2022/Shynamo/DevOpsTraining.git'

This is because the address provided by the container expect port 80 to be used by default, so the port is not specified in the URL. But Git LFS resolve URLs on the host, so the port specification is missing and Git LFS tries to push references on `127.0.0.1:80` instead of `127.0.0.1:2080`.

I did not find a way to fix this inside of the container to that the user won't have anything to change in their configuration. This would be mandatory in production, but for the moment I just need my Git LFS to work. I needed to use these two commands:

    git config lfs.transfer.enablehrefrewrite true
    git config url."http://localhost:2080/".insteadOf "http://localhost/"

I also needed to fix the port in `url` and `pushurl` of my `.git/config`'s `[lfs]` key.

They affected my `.git/config` as follows:

```toml
[core]
  repositoryformatversion = 0
  filemode = true
  bare = false
  logallrefupdates = true
[remote "origin"]
  url = ssh://git@localhost:2022/Shynamo/DevOpsTraining.git
  fetch = +refs/heads/*:refs/remotes/origin/*
[branch "main"]
  remote = origin
  merge = refs/heads/main
[lfs]
  access = basic
  repositoryformatversion = 0 
  url = "http://localhost:2080/Shynamo/DevOpsTraining.git/info/lfs"
  pushurl = "http://localhost:2080/Shynamo/DevOpsTraining.git/info/lfs"
[lfs "transfer"]
  enablehrefrewrite = true
[url "http://localhost:2080/"]
  insteadOf = http://localhost/
```

This would require me to use login/password every time and LFS object is pushed but it is acceptable.

And boom, I have been able to send LFS objects in my GitLab server:

    baptiste:~/Projects/GitHub/DevOpsTraining$ git push
    Username for 'http://localhost:2080': Shynamo
    Password for 'http://Shynamo@localhost:2080': 
    Locking support detected on remote "origin". Consider enabling it with:
      $ git config lfs.http://localhost:2080/Shynamo/DevOpsTraining.git/info/lfs.locksverify true
    Uploading LFS objects: 100% (4/4), 519 KB | 0 B/s, done.                                                                                                                                                   
    Enumerating objects: 24, done.
    Counting objects: 100% (24/24), done.
    Delta compression using up to 16 threads
    Compressing objects: 100% (20/20), done.
    Writing objects: 100% (21/21), 6.12 KiB | 6.12 MiB/s, done.
    Total 21 (delta 8), reused 0 (delta 0), pack-reused 0
    To ssh://localhost:2022/Shynamo/DevOpsTraining.git
      8490f9b..48238aa  main -> main

The downside of this approach is that every user who need to setup their Git config to be able to use Git FLS.

## Automatically start at system reboot

An issue was encountered after rebooting my system: the GitLab image was not automatically starting.

To do so, I used `systemd` following a well written [tutorial](https://linuxhandbook.com/autostart-podman-containers/):

```shell
baptiste:~$ podman generate systemd --new --name gitlab -f
/home/baptiste/container-gitlab.service
baptiste:~$ mkdir ~/.config/systemd/user/
baptiste:~$ mv container-gitlab.service ~/.config/systemd/user/
baptiste:~$ systemctl --user daemon-reload
baptiste:~$ systemctl --user enable container-gitlab.service
Created symlink /home/baptiste/.config/systemd/user/default.target.wants/container-gitlab.service → /home/baptiste/.config/systemd/user/container-gitlab.service.
```

Basically, these commands create a `systemd` config file to automatically start my gitlab container at startup, and then setup the config and enable the service. Here is the content of the `container-gitlab.service` config file:

```toml
# container-gitlab.service
# autogenerated by Podman 3.4.4
# Fri Apr  7 19:43:40 CEST 2023

[Unit]
Description=Podman container-gitlab.service
Documentation=man:podman-generate-systemd(1)
Wants=network-online.target
After=network-online.target
RequiresMountsFor=%t/containers

[Service]
Environment=PODMAN_SYSTEMD_UNIT=%n
Restart=always
TimeoutStopSec=70
ExecStartPre=/bin/rm -f %t/%n.ctr-id
ExecStart=/usr/bin/podman run --cidfile=%t/%n.ctr-id --cgroups=no-conmon --rm --sdnotify=conmon --replace --detach --hostname localhost --publish 20443:443 --publish 2080:80 --publish 2022:22 --name gitlab --volume /home/baptiste/GitLabServer/config:/etc/gitlab --volume /home/baptiste/GitLabServer/logs:/var/log/gitlab --volume /home/baptiste/GitLabServer/data:/var/opt/gitlab --shm-size 256m gitlab/gitlab-ee:latest
ExecStop=/usr/bin/podman stop --ignore --cidfile=%t/%n.ctr-id
ExecStopPost=/usr/bin/podman rm -f --ignore --cidfile=%t/%n.ctr-id
Type=notify
NotifyAccess=all

[Install]
WantedBy=default.target
```

## Security Side Notes

You may wonder why I insisted to install GitLab in user mode. This is because in a production environment, if an attack on a GitLab vulnerability success you don't want the attacker to gain root access on the server.

The attacker will first gain access inside the container. Then, if he manages to escape from the container to the host, he would have ***user*** access. Because GitLab is a software, you can run its container using a user with minimal privileges.

For example, you could:

1. Create a `gitLab` user
2. Only give that user minimum privileges:
   - Create a group `softwares`
   - Add gitlab to that group
   - Give basically no access whatsoever to users or the `software` group
   - Give `gitlab` user RW access only in its home directory, where the data is located
3. Redirect ports 22, 80 and 443 traffic from GitLab to the ports bound in the container using a rootful NGINX in a pod with you GitLab container
4. Encrypt `gitlab`'s home directory

That way, even if the attacker manages to perform two successive attacks, his privilege gain race would result in having only access to `gitlab`'s home directory and OS libs, bins and configs.

This access management can also be done for every other software stored in you server, to protect datas of a service when another service has successfully been attacked.

I did not do this on my PC because I did not need as much security measures as in a production environment, but maybe I will try it later, you never know !
