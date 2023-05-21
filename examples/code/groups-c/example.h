/*! @defgroup group1 The First Group
    @defgroup group2 The Second Group
    @defgroup group3 The Third Group
*/

/*! @ingroup group1
  A function

  \param arg1 the first arg
  \param arg2 the second arg
*/
void example_function_for_c_groups(int arg1, bool arg2);

/*! @ingroup group1
  A struct
*/
struct CGroupExampleStruct {};

/*! @ingroup group2
  Another function

  \param arg1 the first arg
  \param arg2 the second arg
*/
void another_example_function_for_c_groups(int arg1, bool arg2);

/*! @ingroup group2
  A third function

  \param arg1 the first arg
  \param arg2 the second arg
*/
void a_third_example_function_for_c_groups(int arg1, bool arg2);

/*! @ingroup group3
  A fourth function

  \param arg1 the first arg
  \param arg2 the second arg
*/
void a_fourth_example_function_for_c_groups(int arg1, bool arg2);

/*! @defgroup group4 The Fourth Group
    @ingroup group3
    A fifth function

    \param arg1 the first arg
    \param arg2 the second arg
*/
void a_fifth_example_function_for_c_groups(int arg1, bool arg2);
