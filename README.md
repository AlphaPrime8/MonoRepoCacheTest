# Testing docker-chef 

## NOTES
- must add: `/app/chef_test` to start command 

### Docker Clean Commands
- `sudo chmod 666 /var/run/docker.sock`
- `docker stop $(docker ps -a -q)`
- `docker system prune`

### Docker Clean Commands
- `docker build . -f Dockerfile.ssr -t ssr:latest`
- `docker run -e HOST=0.0.0.0 -e PORT=8080 -it -p 8080:8080 --entrypoint sh ssr:latest`

### Hotload Comands
- `cd /home/alphaprime8/RustroverProjects/chef_workspace_test/web/apps/test_ui && wasm-pack build --release --target web --out-dir ../react_app_template/pkg`
- `cd /home/alphaprime8/RustroverProjects/chef_workspace_test/web/apps/react_app_template && npm run build`