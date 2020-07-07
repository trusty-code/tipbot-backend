#!/bin/bash
set -x

set

IP=v2202007102949122169.happysrv.de
PORT=22
DEPLOY=/home/trustify-www

eval "$(ssh-agent -s)" # Start ssh-agent cache
chmod 600 .travis/trustify # Allow read access to the private key
ssh-add .travis/trustify # Add the private key to SSH
if [ "$TRAVIS_BRANCH" == "master" ]; then
  rsync -a --verbose --delete -e "ssh -o StrictHostKeyChecking=no" .travis/tipbot-backend.master.service static data target/release/tipbot-backend trustify-www@$IP:$DEPLOY/master/

#  git config --global push.default matching
#  git remote add deploy ssh://git@$IP:$PORT$DEPLOY_DIR
#  git push deploy master

  # Skip this command if you don't need to execute any additional commands after deploying.
  ssh trustify-www@$IP -p $PORT <<EOF
    sudo /usr/bin/systemctl restart tipbot-backend.master.service
EOF

fi