# Project Judas Data Collector

This tool is designed to make the process of collecting data and writing bugs as easy as possible for testers. This is done by:

1. Collecting data from game logs
2. Displaying it for the user to copy/paste
3. Using that information to populate data in a pre-defined bug template

Currently, users are able to define callback fields (information collected from logs) and manifests (how that information is used).

**You may view examples of both callback fields and manifests in the included files!**

To add a new callback field, simply edit callbackFields.xml.
Currently, the syntax for a callback field is the following
```xml
<callbackField>
	<name>Name of the callback field</name>
	<pattern>Regex to search logs for</pattern>
	<file_name>Name of the log FILE (don't include the folder!)</file_name>
	<hint_text>This text appears when hovering over the name</hint_text>
	<group>
		true/false (deafult: false). Defines if this occurs on multiple lines in a row. Niche, usually should be false.
	</group>
</callbackField>
```


To change the template that gets copied to your clipboard, please view "manifest.json" as an example!
Some things to know there are:
```
*This is bold text*
_This it italic text_
# This is a numbered item
## You can indent these!
* This is a bullet item
** You can also indent these!
```

Make sure to use `\n` to add new lines/line breaks to your template

```json
{ "text": "This text and" },
{ "text": "this text on on the same line!" },
```

```json
{ "text": "This text and \n" },
{ "text": "THIS text go on the different lines!" },
```

If you have anyother questions, please ask `zackary.semancik@lionbridge.com`, or `Zackary Semancik` on the LB-EMAA slack!
