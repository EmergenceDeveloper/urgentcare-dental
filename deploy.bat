@echo off

echo Deploying to Cloudflare...
wrangler pages deploy public --project-name=urgentcare-dental --commit-dirty=true

echo ✨ Deployed!