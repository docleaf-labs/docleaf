/*! @defgroup group1 The First Group
    @defgroup group2 The Second Group
    @defgroup group3 The Third Group
    @defgroup group4 The Fourth Group
    @defgroup group6 The Sixth Group
*/

/*! @ingroup group1
  A function

  \param arg1 the first arg
  \param arg2 the second arg
*/
void group_example_function(int arg1, bool arg2);

/*! @ingroup group1
  A class 
*/
class ExampleClass {
  public:
    ExampleClass();
};

/*! @ingroup group2
  Another function

  \param arg1 the first arg
  \param arg2 the second arg
*/
void group_another_example_function(int arg1, bool arg2);

/*! @ingroup group2
  A third function

  \param arg1 the first arg
  \param arg2 the second arg
*/
void group_a_third_example_function(int arg1, bool arg2);

/*! @ingroup group3
  A fourth function

  \param arg1 the first arg
  \param arg2 the second arg
*/
void group_a_fourth_example_function(int arg1, bool arg2);

/*! @ingroup group3
  A fifth function

  \param arg1 the first arg
  \param arg2 the second arg
*/
void group_a_fifth_example_function(int arg1, bool arg2);

/*! @defgroup group5 The Fifth Group
    @ingroup group4
    A fifth function

    \param arg1 the first arg
    \param arg2 the second arg
*/
void group_a_sixth_example_function(int arg1, bool arg2);

/*! @ingroup group6
  A seventh function

  \param arg1 the first arg
  \param arg2 the second arg
*/
void group_a_seventh_example_function(int arg1, bool arg2);
