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
dft
  [options]
  [--in=<filename>]  [--iformat=<format>]
  [--out=<filename>] [--oformat=<format>]
  [--verbose=<level>]

options:
  -U                  Evaluate the instructions in reverse to undo the result
  --in=<filename>     File to consume as input, if not present defaults to stdin
  --iformat=<format>  Overwrites the format assumed by the in filename extension,
                      required when reading from stdin
                        - json
                        - csv
  --out=<filename>    File to write as output, if not present defaults to stdout
  --oformat=<format>  Overwrites the format assumed by the out filename
                      extension, required when writing to stdout
                        - json
                        - csv
                        - sql
  --verbose=<level>   Level of verbosity, from 0 to 5


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
    - EQUAL
    - GREATER
    - LESSER
    - DIFFERENT

type:
  attribute data type
    - number
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
DISTINCT  <field>
IGNORE    <field[,field...]>
FILTER    <expression>                <value>
DEFINE    <formula>                   AS      <op>
ALIAS     <field>                     AS      <field>
RENAME    <field>                     TO      <field>
APPLY     <op[,op...]>                TO      <field>
MERGE     <field[,field...]>          AS      <field>
COERCE    <field[,field...]>          TYPED   <type>        RESCUE    <value>
ADD       <field[,field...]>          TYPED   <type>        [DEFAULT   <field>]
