# Set Up A GitLab Runner

As I like to make things harder but more secure, I tried to install GitLab Runner in a rootless Podman container.

## GitLab Runner On Host

To install GitLab Runner on my host, I simply followed GitLab's [documentation](https://docs.gitlab.com/runner/install/).

In short, I simply ran the following commands for a Debian (`.deb`) installation:

```cmd
arch=$(dpkg --print-architecture)
curl -LJO "https://gitlab-runner-downloads.s3.amazonaws.com/latest/deb/gitlab-runner_${arch}.deb"
dpkg -i gitlab-runner_${arch}.deb
```

TODO: Redaction

## Fixes

--network = host in the config

- Explain the Issue

Try to put an IP in /etc/host and use it instead of 127.0.0.1.
