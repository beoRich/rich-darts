# Development

Your new jumpstart project includes basic organization with an organized `assets` folder and a `components` folder. 
If you chose to develop with the router feature, you will also have a `views` folder.

### Tailwind
1. 
Windows + Linux Bash:
Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
Linux fish shell:
https://github.com/jorgebucaran/nvm.fish?tab=readme-ov-file
then 
linux fish: nvm use latest
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation REQUIRES TAILWIND 3, DOES NOT WORK with TAILWIND 4
3. install daisyUi npm i -D daisyui@latest
4. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve --platform web
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

