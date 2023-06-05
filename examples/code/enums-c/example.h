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

/** Docs */
enum MultiLineInitializer {
        /** A Docs */
        ENTRY_A = BIT(0),
        /** B Docs */
        ENTRY_B  = BIT(1),
        /** C Docs */
        ENTRY_C = BIT(2),
        /** D Docs */
        ENTRY_D = BIT(3),
        /** All Docs */
        ENTRY_ALL = ENTRY_A | ENTRY_B |
                    ENTRY_C | ENTRY_D,
};
