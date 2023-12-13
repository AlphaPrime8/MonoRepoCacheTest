# Cargo Chef for Monorepos

This is a proof of concept repo for efficient use of docker caching layers in a rust monorepo.
Existing examples, and the documentation for cargo-chef does not apply intuitively to monorepos (at least for me lol).

Credit to LukeMathWalker for his work, please star his repo: https://github.com/LukeMathWalker/cargo-chef

## Build Test Process

### Docker Build Commands
- `docker build . -f Dockerfile.app1 -t app1:latest`
- `docker run -it --entrypoint sh app1:latest` -> `./runner`

## TODO
- Try w/ and w/out docker ignore target...







## IGNORE ALL BELOW, its copy pasta'd and will be updated shortly
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