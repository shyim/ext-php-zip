# PHP Zip extension

This extension allows to handle zip archives in memory without the need of a temporary file.

```php
$a = file_get_contents('test.zip');
$test = new RustZip($a);
// is file in zip
$test->has('README.md');
// get file content from zip
$test->get('README.md');
// get all files in zip
$test->files();
```
