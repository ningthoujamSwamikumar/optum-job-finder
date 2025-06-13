# optum-job-finder
Finds job on optum career by job number

This script only works for static site or content.
Dynamic contents load by scripts in the browser can't be fetch using scraper and reqwest.
For that we must use browser automation using like fantoccini or third party services.

And to install and run browser and webdriver in the github actions runner will specially required load of resource,
and for this small job, I don't think its feasible to keep running by cron schedule. 