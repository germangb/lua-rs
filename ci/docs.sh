#!/bin/bash

DOCS_DIR=.doc/

cargo doc
mv target/doc $DOCS_DIR

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

git -C $DOCS_DIR init && \
    git -C $DOCS_DIR remote add origin https://github.com/germangb/lua-rs.git && \
    git -C $DOCS_DIR checkout -b gh-pages && \
    git -C $DOCS_DIR add -A && \
    git -C $DOCS_DIR commit -m "Publish docs" && \
    git -C $DOCS_DIR push origin gh-pages --force
