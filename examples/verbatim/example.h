/*! The function with verbatim tags

  \verbatim
  This content should
     appear verbatim
  \endverbatim
*/
void example_function_verbatim(int arg1, bool arg2);

/*! The function with code block

        This content should
           appear as a code block
*/
void example_function_code_block(int* a, int* b, int* c);

/*! The function with rst block

    \verbatim embed:rst
    This is ``ReStructuredText`` and should be **formatted** as such
    \endverbatim
*/
void example_function_rst(int* a, int* b, int* c);

/*! The function with rst block

    \rst
    This is ``ReStructuredText`` and should be **formatted** as such
    \endrst
*/
void example_function_rst_alias(int* a, int* b, int* c);

/// The function with rst block with leading slashes
///
/// \verbatim embed:rst:leading-slashes
/// This is ``ReStructuredText`` and should be **formatted** as such
/// \endverbatim
///
void example_function_rst_leading_slashes(int* a, int* b, int* c);

/*! The function with rst block with leading asterisk
 * 
 *  \verbatim embed:rst:leading-asterisk
 *  This is ``ReStructuredText`` and should be **formatted** as such
 *  \endverbatim
 */
void example_function_rst_leading_asterisk(int* a, int* b, int* c);

/*! The function with inline rst
 * 
 *  This is inline rst with formating \inlinerst *emphasis* **strong emphasis** ``inline literal`` \endrst which
 *  should be displayed properly.
 */
void example_function_rst_inline(int* a, int* b, int* c);

