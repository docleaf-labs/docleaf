/*! Example enum in the C domain */
enum ExampleCEnum
{
    kEntry1 = 0,     //!< First entry
    kEntry2 = 4,     //!< Second
    kEntry3 = 8      //!< And third
};

/*! @defgroup anon_enum_group Anon Enum Group */
/*! @ingroup anon_enum_group

Anonymous enum */
enum
{
    kAnonEntry1 = 0,     //!< First entry
    kAnonEntry2,         //!< Second
    kAnonEntry3          //!< And third
};
