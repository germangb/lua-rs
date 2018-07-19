#!/bin/bash

cargo doc
mv target/doc doc/
cd doc/

cat docs/index.html << EOF
<!DOCTYPE html>
<html>
<head>
<meta http-equiv="refresh" content="0; url=lua/index.html" />
</head>
<body>
    You are being redirecting to the <a href="lua/index.html">documentation page</a>...
</body>
</html>
EOF

git init && \
    git remote add origin https://github.com/germangb/lua-rs.git && \
    git checkout -b gh-pages && \
    git add -A && \
    git commit -m "Publish docs" && \
    git push origin gh-pages --force
