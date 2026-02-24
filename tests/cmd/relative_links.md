```console
$ cobalt -v build --destination _dest
DEBUG: Using config file `./_cobalt.yml`
Building from `.` into `[CWD]/_dest`
DEBUG: glob converted to regex: Glob { glob: "**/.*", re: "(?-u)^(?:/?|.*/)//.[^/]*$", opts: GlobOptions { case_insensitive: false, literal_separator: true, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursivePrefix, Literal('.'), ZeroOrMore]) }
DEBUG: glob converted to regex: Glob { glob: "**/_*", re: "(?-u)^(?:/?|.*/)_[^/]*$", opts: GlobOptions { case_insensitive: false, literal_separator: true, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursivePrefix, Literal('_'), ZeroOrMore]) }
DEBUG: built glob set; 5 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 2 regexes
DEBUG: Loading data from `./_data`
DEBUG: glob converted to regex: Glob { glob: "**/.*", re: "(?-u)^(?:/?|.*/)//.[^/]*$", opts: GlobOptions { case_insensitive: false, literal_separator: true, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursivePrefix, Literal('.'), ZeroOrMore]) }
DEBUG: glob converted to regex: Glob { glob: "**/_*", re: "(?-u)^(?:/?|.*/)_[^/]*$", opts: GlobOptions { case_insensitive: false, literal_separator: true, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursivePrefix, Literal('_'), ZeroOrMore]) }
DEBUG: built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 2 regexes
DEBUG: Loading snippets from `./_includes`
Generating posts/folder/post.html
global_collection collections
Normalizing link: bear.png with url_path: posts/folder/post.html and base_url https://cobalt-org.github.io, document_url https://cobalt-org.github.io/posts/folder/post.html
Normalizing link: ../elephant.png with url_path: posts/folder/post.html and base_url https://cobalt-org.github.io, document_url https://cobalt-org.github.io/posts/folder/post.html
Not normalizing link: /posts/folder/crocodile.png with url_path: posts/folder/post.html and base_url https://cobalt-org.github.io, document_url https://cobalt-org.github.io/posts/folder/post.html
Not normalizing link: http://example.com/chimpanzee.png with url_path: posts/folder/post.html and base_url https://cobalt-org.github.io, document_url https://cobalt-org.github.io/posts/folder/post.html
Normalizing link: bear.png with url_path: posts/folder/post.html and base_url https://cobalt-org.github.io, document_url https://cobalt-org.github.io/posts/folder/post.html
Normalizing link: ../elephant.png with url_path: posts/folder/post.html and base_url https://cobalt-org.github.io, document_url https://cobalt-org.github.io/posts/folder/post.html
Not normalizing link: /posts/folder/crocodile.png with url_path: posts/folder/post.html and base_url https://cobalt-org.github.io, document_url https://cobalt-org.github.io/posts/folder/post.html
Not normalizing link: http://example.com/chimpanzee.png with url_path: posts/folder/post.html and base_url https://cobalt-org.github.io, document_url https://cobalt-org.github.io/posts/folder/post.html
Normalizing link: parrot.png with url_path: posts/folder/post.html and base_url https://cobalt-org.github.io, document_url https://cobalt-org.github.io/posts/folder/post.html
Normalizing link: ../squirrel.png with url_path: posts/folder/post.html and base_url https://cobalt-org.github.io, document_url https://cobalt-org.github.io/posts/folder/post.html
Not normalizing link: /posts/folder/wolf.png with url_path: posts/folder/post.html and base_url https://cobalt-org.github.io, document_url https://cobalt-org.github.io/posts/folder/post.html
Not normalizing link: http://example.com/zebra.png with url_path: posts/folder/post.html and base_url https://cobalt-org.github.io, document_url https://cobalt-org.github.io/posts/folder/post.html
Normalizing link: chicken.png with url_path: posts/folder/post.html and base_url https://cobalt-org.github.io, document_url https://cobalt-org.github.io/posts/folder/post.html
Normalizing link: ../dog.png with url_path: posts/folder/post.html and base_url https://cobalt-org.github.io, document_url https://cobalt-org.github.io/posts/folder/post.html
Not normalizing link: /posts/folder/gorilla.png with url_path: posts/folder/post.html and base_url https://cobalt-org.github.io, document_url https://cobalt-org.github.io/posts/folder/post.html
Not normalizing link: http://example.com/horse.png with url_path: posts/folder/post.html and base_url https://cobalt-org.github.io, document_url https://cobalt-org.github.io/posts/folder/post.html
Normalizing link: lion.png with url_path: posts/folder/post.html and base_url https://cobalt-org.github.io, document_url https://cobalt-org.github.io/posts/folder/post.html
Normalizing link: ../tiger.png with url_path: posts/folder/post.html and base_url https://cobalt-org.github.io, document_url https://cobalt-org.github.io/posts/folder/post.html
Not normalizing link: /posts/folder/cat.png with url_path: posts/folder/post.html and base_url https://cobalt-org.github.io, document_url https://cobalt-org.github.io/posts/folder/post.html
Not normalizing link: http://example.com/giraffe.png with url_path: posts/folder/post.html and base_url https://cobalt-org.github.io, document_url https://cobalt-org.github.io/posts/folder/post.html
Generating page with pagination index.html
Generating index.html
global_collection paginator
Build successful

```
