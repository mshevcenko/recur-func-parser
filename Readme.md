### General recursive function parser

This parser is built for general recursive function / partial recursive function parsing.
After all functions are parsed, they will be used for calculations.
Anyone will be able to practice creating a general recursive function using this parser.

For more information about general recursive function: 
 - https://en.wikipedia.org/wiki/General_recursive_function

zero function mark: "z"  
successor function mark: "s"  
projection function mark: "p&lt;argument number&gt;.&lt;argument quantity&gt;"  
composition function mark: "(&lt;func&gt; : &lt;func&gt;, ... , &lt;func&gt;)"  
primitive function mark: "[&lt;func&gt; , &lt;func&gt;]"  
minimization function mark: "{&lt;func&gt; , &lt;integer&gt;}"  

Example of recursive function: "addition = [p1.1, (s:p3.3)];"  
Expected use case:  
&emsp;Input: addition 1 3  
&emsp;Output: 4  