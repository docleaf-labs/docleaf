/*! The first function

  With a pargraph below the first one.

  And another one after that.

  \param arg1 the first arg
  \param arg2 the second arg
*/
void example_function(int arg1, bool arg2);

/*! Parameters with directions
 *  \param[out]     a output
 *  \param[in]      b input
 *  \param[in, out] c input and output
 */
void example_function_directions(int* a, int* b, int* c);

/*! Function with return type
 *  \param a input
 *  \retval 0 if successful
 */
int example_function_with_retval(int a);

/*! Function with two retvals
 *  \param a input
 *  \retval 0 if successful
 *  \retval 1 if error
 */
int example_function_with_retvals(int a);

/*! Function with return
 *  \param a input
 *  \return the number the user wants
 */
int example_function_with_return(int a);

/*! Function with return type
 *  \param a input
 *  \returns the number that the user wants
 */
int example_function_with_returns(int a);
