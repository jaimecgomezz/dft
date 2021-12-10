dft
--------------------------------------------------------------------------------
data files transformer cli built with rust


notice
--------------------------------------------------------------------------------
this readme is almost a wishlist and general description of the project, is yet
to be implemented, so please, ask the developer about its day rather than why it
isn't working yet

  - the developer


usage
--------------------------------------------------------------------------------
dft [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -u, --undo       Reverse evaluate
    -V, --version    Prints version information
    -v, --verbose    Verbose mode (-v, -vv, -vvv, ...)

OPTIONS:
    -f, --from <from>                    Input format, see availables
    -i, --input <input>                  Input file, defaults to stding
    -z, --instructions <instructions>    Instructions file [default: instructions.dft]
    -o, --output <output>                Output file, defaults to stdout
    -t, --to <to>                        Output format, see availables


concepts
--------------------------------------------------------------------------------
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
    - EQUALS
    - GREATER
    - EQGREATER
    - LESSER
    - EQLESSER
    - DIFFERS

type:
  attribute data type
    - integer
    - float
    - boolean
    - string
      | date
      | time
      | datetime
      | uri
      | email
      | uuid


syntax
--------------------------------------------------------------------------------
DISTINCT  <field[,field...]>
IGNORE    <field[,field...]>
ALIAS     <field>               TO        <field>
RENAME    <field>               TO        <field>
MERGE     <field[,field...]>    TO        <field>
FILTER    <field[,field...]>    THAT      <expression>    <value>
COERCE    <field[,field...]>    TYPED     <type>          RESCUE    <value>
ADD       <field[,field...]>    TYPED     <type>          DEFAULT   <value>


instructions
--------------------------------------------------------------------------------
DISTINCT:
  filters out records with same value on <field>

IGNORE:
  filters out the <field> itself
  this operation makes the result undoable

FILTER:
  filters out records that fail to match its value against <value>

ALIAS:
  aliases a given <field> to also be refered as <field>, useful in conjuction
  with APPLY

RENAME:
  changes the current <field> name to <field>

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
