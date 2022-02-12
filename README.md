# Packall

Packall is a command program that eats all the files you feed and keeps them organized, first in the belly, after in the body, for future searches.

# Technology

### Project features

* Import files.
* Does not keep repeated files.
* Makes indexes for fast search.
* Uses associated software present in path for conversions to text.
* The indexes are made from the converted texts.

### Files structure

The files are kept in subdirectories inside the body root directory. The main factor of
organization is the SHA256 checksum of the origin file. Each file is kept inside of three
nested directories. The first directory name is the three first characters from the
checksum. The second directory name is made starting with the forty character from the
checksum until the sixth. Finally, the last and third directory name is made with all the
checksum. Inside the third and last directory our file is stored. The name of the stored
file is made with the start of "org-", that comes from "origin", plus a sequence of
eighteen random characters and his extension.
