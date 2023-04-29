# NGINX Proxy

Table of contents:

[TOC]

Remember that I used the following command to run GitLab ?

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

Well, I cannot click in my issue and merge request links on my [GitLab local server](setting_up_gitlab.md) because of non-standard ports redirection. This was also the cause of LFS issues, so it is time to fix this by making the port redirection transparent.

## NGINX Proxy For HTTP And HTTPS

To fix this issue, I will use a rootful proxy [NGINX](https://www.nginx.com/) to forward ports based on their domain name, with the following configuration in `/root/nginx.conf`:

```nginx
events {
  worker_connections  4096;
}

http {
 server {
   listen 127.0.0.1:80;
   listen [::1]:80 ipv6only=on;

   server_name shynamo-gitlab;

   location / {
       proxy_pass http://127.0.0.1:2080;
       proxy_set_header Host $host;
   }
 }
 server {
   listen 127.0.0.1:443;
   listen [::1]:443 ipv6only=on;

   server_name shynamo-gitlab;

   location / {
       proxy_pass http://127.0.0.1:20443;
       proxy_set_header Host $host;
   }
 }
}
```

No need to put the NGINX container in a pod with my GitLab container as one is rootful and to other rootless.

```cmd
$ sudo podman run \
    --detach \
    --restart always \
    --network host \
    -v /root/nginx.conf:/etc/nginx/nginx.conf \
    --name nginx \
    nginx
```

Then, I made the container restart at startup:

```cmd
baptiste:~$ sudo podman generate systemd --new --name nginx -f
/home/baptiste/container-nginx.service
baptiste:~$ sudo cp container-nginx.service /etc/systemd/system/
baptiste:~$ sudo systemctl daemon-reload
baptiste:~$ sudo systemctl enable container-nginx
Created symlink /etc/systemd/system/default.target.wants/container-nginx.service → /etc/systemd/system/container-nginx.service.
baptiste:~$ sudo systemctl status container-nginx
○ container-nginx.service - Podman container-nginx.service
     Loaded: loaded (/etc/systemd/system/container-nginx.service; enabled; vendor preset: enabled)
     Active: inactive (dead)
       Docs: man:podman-generate-systemd(1)
```

Finally, I need to [rename my GitLab's hostname](#rename-gitlab-hostname) into `shynamo-gitlab` to make those HTTP and HTTPS completely transparent.

## SSH Config

For the SSH port redirection, I simply edited my `~/.ssh/config` with the following:

```cmd
# GitLab Local Installation
Host shynamo-gitlab
  PreferredAuthentications publickey
  IdentityFile ~/.ssh/id_rsa.pub
  HostName localhost
  User git
  Port 2022
```

This uses port 2022 by default for the user `git` on `shynamo-gitlab` (resolved as `localhost`), which is exactly what I need.

## Rename GitLab Hostname

When running my GitLab container, I used the option `--hostname localhost`. With the NGINX's server I need to use `shynamo-gitlab` instead.

If possible, I would like to use the same container instead of re-creating one from the image.

TODO: gitlab config URL

## Remove Trailing Slash

GitLab job adds trailing `/` to the Git repository when trying to pull it, which results in 404 error.

Fix this in NGINX config:

```nginx
events {
  worker_connections  4096;
}

http {
 server {
   listen localhost:80;
   listen [::1]:80 ipv6only=on;

   server_name shynamo-gitlab;

   location ~ (?<no_slash>.*)/$ {
       proxy_pass http://localhost:2080$no_slash;
       proxy_set_header Host $host;
   }
 }
 server {
   listen localhost:443;
   listen [::1]:443 ipv6only=on;

   server_name shynamo-gitlab;

   location ~ (?<no_slash>.*)/$ {
       proxy_pass http://localhost:20443$no_slash;
       proxy_set_header Host $host;
   }
 }
}
```
