/** @type {import("snowpack").SnowpackUserConfig } */
module.exports = {
    mount: {
        src: "/",
        static: "/",
    },
    plugins: ["@snowpack/plugin-react-refresh", "@snowpack/plugin-typescript"],
}
