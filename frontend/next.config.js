/** @type {import('next').NextConfig} */
const isExport = process.env.NEXT_EXPORT === '1';

const nextConfig = isExport
  ? { output: 'export' }
  : {
      async rewrites() {
        return [
          {
            source: '/api/:path*',
            destination: 'http://localhost:8080/api/:path*',
          },
        ];
      },
    };

module.exports = nextConfig;
