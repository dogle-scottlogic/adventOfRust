/** @type {import('next').NextConfig} */
const nextConfig = {}

module.exports = {
    reactStrictMode: true,
    webpack: function (config) {
        config.experiments = { asyncWebAssembly: true, layers: true };
        return config;
    }
}
