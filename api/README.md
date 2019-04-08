# Insulter API
Manage insults via a REST api. Create and Update via the management tagged endpoints. 
Generate insults via the user tagged endspoints. The API is stored as an 
[Open API v3](https://github.com/OAI/OpenAPI-Specification) spec.

## Info
Below are some extra details around some of the concepts in the API that may need more
explanation.

### Cursors
Pagination in the API is loosely based on [twitters cursor based](https://developer.twitter.com/en/docs/ads/general/guides/pagination.html) 
implementation. If a **GET** call to the query endpoint for insults results in 
returned records, then there will be `previous` and `next` properties in the returned
JSON which will contain URL's to retrieve the next batch of results. The direction
of the ordering of the records will be encoded into the cursor.
 