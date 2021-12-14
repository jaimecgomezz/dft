dft
--------------------------------------------------------------------------------
data files transformer cli


notice
--------------------------------------------------------------------------------
this readme is almost a wishlist and general description of the project, is yet
to be implemented, so please, ask the developer about its day rather than why it
isn't working yet

  - the developer


help
--------------------------------------------------------------------------------
data files transformer cli

USAGE:
    dft [FLAGS] [OPTIONS]

FLAGS:
    -h, --help         Prints help information
        --overwrite    Overwrite output
    -u, --undo         Reverse evaluate?
    -V, --version      Prints version information
    -v, --verbose      Verbose mode (-v, -vv, -vvv, ...)

OPTIONS:
    -F, --format <format>                Input and output format, overwrites <from> and <to>
    -f, --from <from>                    Input format, see availables
    -i, --input <input>                  Input file, defaults to stdin
    -z, --instructions <instructions>    Instructions file [default: instructions.dft]
    -l, --logger <logger>                Logger output, defaults to stdout
    -o, --output <output>                Output file, defaults to stdout
    -t, --to <to>                        Output format, see availables


concepts
--------------------------------------------------------------------------------
record:
  an element in the data collection

value:
  plain value

field:
  name of an attribute

formula:
  an inline algebraic expression

op:
  name of a operation, so it can be later referred to

expression:
  comparison operation
    - equals
    - greater
    - eqgreater
    - lesser
    - eqlesser
    - differs

type:
  attribute data type
    - number
    - boolean
    - string

formats:
  string value format
    - date
    - time
    - datetime
    - uri
    - email
    - uuid

actions:
  reaction to an event
    - discard
    - notify
    - halt


syntax
--------------------------------------------------------------------------------
DISTINCT  <field[,field...]>
IGNORE    <field[,field...]>
ALIAS     <field>               TO        <field>
RENAME    <field>               TO        <field>
MERGE     <field[,field...]>    TO        <field>
FILTER    <field[,field...]>    MATCHING  <expression>    <value>
VALIDATE  <field>               MATCHING  <format>        OR        <action>
COERCE    <field[,field...]>    TYPED     <type>          RESCUE    <value>
ADD       <field[,field...]>    TYPED     <type>          DEFAULT   <value>


instructions
--------------------------------------------------------------------------------
DISTINCT:
  filters out records with same value on <field>

IGNORE:
  filters out the <field> itself
  this operation makes the result undoable

ALIAS:
  aliases a given <field> to also be refered as <field>, useful in conjuction
  with APPLY

RENAME:
  changes the current <field> name to <field>

MERGE:
  merges <fields> into <field>

FILTER:
  filters out records that fail to match its value against <value>

VALIDATE:
  validates that <field> conforms to <format> and execs <action> on failure

COERCE:
  parses <field> into the specified <type> and rescues failure with <value>

ADD:
  adds <field> to records with <type> and defaults to <value>


wishlits
--------------------------------------------------------------------------------

DEFINE
----------
  defines custom <formula> as <op> the be applied to data with <type>

  DEFINE    <formula>             AS        <op>            FOR       <type>

APPLY
----------
  evaluates <field> with <op> and rescues failure with <value>

  APPLY     <op[,op...]>          TO        <field>         RESCUE    <value>
