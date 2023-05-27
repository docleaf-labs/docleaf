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
struct CGroupExampleStruct {

  //! Function pointer struct member
  void (*fnc_ptr)(struct CGroupOtherStruct *sync, int *info);

  //! Another func pointer
  void (*state_changed)(struct bt_le_per_adv_sync *sync,
                              const struct bt_le_per_adv_sync_state_info *info);
};

/*! @ingroup group1
  Another struct
*/
struct CGroupOtherStruct {};

/*! @ingroup group1
  A define macro for group 1
*/
#define GROUP_C_NAME "group-c"

/*! @ingroup group1
  A define macro with args for group 1
*/
#define GROUP_C_HAS_STATUS(item_id, status) \
	has_status(item_id, status)

/*! @ingroup group1
  A simple typedef
*/
typedef uint32_t my_type_def;

/*! @ingroup group1
  A less simple typedef
*/
typedef void (*func_ptr)(struct mytype *value, int err, uint8_t id);

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
