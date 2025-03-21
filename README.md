# About

A webapp to track a Darts Game, written in Rust for frontend and backend via dioxus.
Currently under development


### Dioxus 

Install dixous cli
Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve --platform web
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```


### Tailwind
1. Windows + Linux Bash:
Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
Linux fish shell: https://github.com/jorgebucaran/nvm.fish?tab=readme-ov-file then linux fish: nvm use latest)
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation REQUIRES TAILWIND 3, DOES NOT WORK with TAILWIND 4
3. install daisyUi npm i -D daisyui@latest
4. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```


### DockerImage
1. docker build . -t <tag>
2. docker run -t <tag> 
3. docker container ls  
3. docker inspect <containerId>
4. 
search for  NetworkSettings.Networks
or 
docker inspect \
        -f '{{range.NetworkSettings.Networks}}{{.IPAddress}}{{end}}' <containerId>
for local IP
5. webbrowser <localIp>:8080 


### Deployment (Currently)
1. Login to digital Oceean
2. Create/Use a container registry
3. push image to container registry
4. Create/Use a droplet
5. ssh login into droplet 
6. setup docker inside the droplet
7. docker login into container registry
8. pull image from container registry
9. docker run -d -p 80:8080 registry.digitalocean.com/rich-registry/rich-darts
10. access ip4 address of that droplet




