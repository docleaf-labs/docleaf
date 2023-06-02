/*! @defgroup cgroup1 The First Group
    @defgroup cgroup2 The Second Group
    @defgroup cgroup3 The Third Group
*/

/*! @ingroup cgroup1
  A function

  \param arg1 the first arg
  \param arg2 the second arg
*/
void example_function_for_c_groups(int arg1, bool arg2);


/*! @ingroup cgroup1
  A struct with nested struct
*/
struct NestedStruct {
  //! Documentation
  int32_t id;
};

/*! @ingroup cgroup1
  A struct with nested struct
*/
struct OuterStruct {
  //! Documentation
  struct NestedStruct nested;
};


/*! @ingroup cgroup1
  A struct
*/
struct CGroupExampleStruct {

  //! Function pointer struct member
  void (*fnc_ptr)(struct CGroupOtherStruct *sync, int *info);

  //! Another func pointer
  void (*state_changed)(struct bt_le_per_adv_sync *sync,
                              const struct bt_le_per_adv_sync_state_info *info);
};

/*! @ingroup cgroup1
  Another struct
*/
struct CGroupOtherStruct {
  //! First field
  uint32_t first_field;

  /*! An anonymous union */
  union {
    //! Union entry 1
    uint32_t a_id : 3;
    //! Union entry 2
    int32_t b_id : 3;
  };

  //! Middle field
  uint32_t middle_field;

  /*! Another anonymous union */
  union {
    //! Union entry 1
    uint32_t c_id : 3;
    //! Union entry 2
    int32_t d_id : 3;
    //! Union entry 3
    int32_t e_id : 3;
  };

  //! Last field
  uint32_t last_field;
};

/*! @ingroup cgroup1
  A define macro for group 1
*/
#define GROUP_C_NAME "group-c"

/*! @ingroup cgroup1
  A define macro with args for group 1
*/
#define GROUP_C_HAS_STATUS(item_id, status) \
	has_status(item_id, status)

/*! @ingroup cgroup1
  A simple typedef
*/
typedef uint32_t my_type_def;

/*! @ingroup cgroup1
  A less simple typedef
*/
typedef void (*func_ptr)(struct mytype *value, int err, uint8_t id);

/*! @ingroup cgroup1
  A static inline function
*/
static inline uint32_t get_end(uint32_t index)
{
  return 0;
}

/*! @ingroup cgroup1
  A named union
*/
union NamedUnion {
  //! a_id entry
  uint32_t a_id  : 3;
  //! b_id entry
  uint32_t b_id  : 8;
};


/*! @ingroup cgroup1
  Anonymous enum at the top level
*/
enum {
      MY_ENTRY_1 = 0x00,
      MY_ENTRY_2 = 0x01,
};

/*! @ingroup cgroup1
  Define with the same name as the anonymous enum
 */
#define MY_ENTRY_1(_a, _b) \
        OTHER_MACROS(_a, SOMETHING, ELSE, _b)


/*! @ingroup cgroup1
  Named union with struct inner
 */
union union_with_struct{
	struct {
		/** Entry a */
		uint32_t a : 7;
		/** Entry b */
		uint32_t b : 1;
	};
	/** Raw value */
	uint32_t raw_value;
};

/*! @ingroup cgroup1
  Struct with inline named union member
 */
struct struct_with_union {
  /** Node */
  union named_union {
    /// Docs
    struct a_t *a;
    /// Docs
    struct b_t *b;
    /// Docs
    struct c_t *c;
  }
  /// Docs
  named_union_field;

};

/*! @ingroup cgroup2
  Another function

  \param arg1 the first arg
  \param arg2 the second arg
*/
void another_example_function_for_c_groups(int arg1, bool arg2);

/*! @ingroup cgroup2
  A third function

  \param arg1 the first arg
  \param arg2 the second arg
*/
void a_third_example_function_for_c_groups(int arg1, bool arg2);

/*! @ingroup cgroup3
  A fourth function

  \param arg1 the first arg
  \param arg2 the second arg
*/
void a_fourth_example_function_for_c_groups(int arg1, bool arg2);

/*! @defgroup cgroup4 The Fourth Group
    @ingroup cgroup3
    A fifth function

    \param arg1 the first arg
    \param arg2 the second arg
*/
void a_fifth_example_function_for_c_groups(int arg1, bool arg2);
