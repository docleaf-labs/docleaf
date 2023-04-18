/*! Description of example class
*/
class ReferenceExample
{
public:

    /*! The first method

      \param arg1 the first arg
      \param arg2 the second arg
    */
    void method_1(int arg1, bool arg2);

};

/*! Description of example class
*/
class OtherReferenceExample
{
public:

    /*! The first method

      \param reference the first arg
    */
    void method(OtherReferenceExample reference);

};

/*! Description of doc reference example class
*/
class DocReferenceExample
{
public:

    /*! This refers to OtherReferenceExample in its docs

      \param reference the first arg
    */
    void method(OtherReferenceExample reference);

};
