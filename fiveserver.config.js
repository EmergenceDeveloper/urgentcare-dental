const fs = require('fs');
const path = require('path');

module.exports = {
  root: 'public',
  middleware: [
    (req, res, next) => {
      const url = req.url;
      
      // Fast path: if has extension (except .html), skip everything
      if (url.includes('.') && !url.endsWith('.html')) {
        return next();
      }
      
      // Redirect .html extension to extensionless
      if (url.endsWith('.html') && !url.endsWith('index.html')) {
        res.writeHead(301, { Location: url.slice(0, -5) });
        res.end();
        return;
      }
      
      // If trailing slash (and not root), check folder
      if (url.endsWith('/') && url !== '/') {
        // Only check if index.html exists - single fast check
        if (!fs.existsSync(path.join(__dirname, 'public', url, 'index.html'))) {
          res.writeHead(301, { Location: url.slice(0, -1) });
          res.end();
          return;
        }
        return next();
      }
      
      // No extension, no slash - add .html
      if (url !== '/') {
        req.url = url + '.html';
      }
      
      next();
    }
  ]
}

/*module.exports = {
  root: 'public',
  middleware: [
    (req, res, next) => {
      // Redirect trailing slashes to non-slash (external redirect)
      if (req.url.endsWith('/') && req.url !== '/') {
        const cleanUrl = req.url.slice(0, -1);
        res.writeHead(301, { Location: cleanUrl });
        res.end();
        return;
      }
      
      // If URL has no extension, rewrite to .html (internal)
      if (!req.url.includes('.') && req.url !== '/') {
        req.url = req.url + '.html';
      }
      
      next();
    }
  ]
}*/

/*module.exports = {
  root: 'public',
  middleware: [
    (req, res, next) => {
      // Redirect trailing slashes to non-slash
      if (req.url.endsWith('/') && req.url !== '/') {
        req.url = req.url.slice(0, -1);
      }
      
      // If URL has no extension and isn't root
      if (!req.url.includes('.') && req.url !== '/') {
        req.url = req.url + '.html';
      }
      
      next();
    }
  ]
}*/