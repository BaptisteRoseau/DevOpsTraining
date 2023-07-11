#!/bin/bash
arch=$(dpkg --print-architecture)
curl -LJO "https://gitlab-runner-downloads.s3.amazonaws.com/latest/deb/gitlab-runner_${arch}.deb"
sudo dpkg -i gitlab-runner_${arch}.deb
sudo gitlab-runner start
