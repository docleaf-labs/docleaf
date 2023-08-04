// This include is not recognised by Doxygen and so Doxygen
// doesn't add a 'refid' to the 'includes' XML tag for it even
// though the compound.xsd suggests it 'includes' always as
// a 'refid' attribute. This sample is here to make sure we
// handle it properly.
#include "my_other_file.h"

/*! The first function

  With a pargraph below the first one.

  And another one after that.

  \param arg1 the first arg
  \param arg2 the second arg
*/
void includes_example_function(int arg1, bool arg2);
