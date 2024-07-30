/** @type {import('next').NextConfig} */
const nextConfig = {
    reactStrictMode: false,
    webpack: (config, options) => {
        // config.experiments.syncWebAssembly = true
        // config.experiments.asyncWebAssembly = true
        return config
    }
};

export default nextConfig;
