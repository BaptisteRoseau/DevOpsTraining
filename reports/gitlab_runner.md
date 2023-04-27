# Setup a GitLab Runner

As I like to make things harder, I try to install GitLab Runner in a rootless podman container.

First, I needed to 

```cmd
podman run --detach   --hostname shynamo-gitlab  --publish 9292:9292  --publish 20443:443 --publish 2080:80 --publish 2022:22   --name gitlab  --network gitlab-net --restart always   --volume $GITLAB_HOME/config:/etc/gitlab   --volume $GITLAB_HOME/logs:/var/log/gitlab   --volume $GITLAB_HOME/data:/var/opt/gitlab   --shm-size 256m gitlab-ee:latest 
```

## Forwarding A New Port

9292

