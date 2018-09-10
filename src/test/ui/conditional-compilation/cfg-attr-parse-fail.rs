// Not having the comma after the predicate in `cfg_attr` is an error.

#[cfg_attr(all())] //~ error: expected `,`, found `)`
struct CfgAttrNoCommaAfterConfigurationPredicate;

#[cfg_attr()] //~ error: expected identifier, found `)`
struct CfgAttrNoConfigurationPredicate;