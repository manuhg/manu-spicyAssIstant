# Spice AI Take-Home Assessment

Welcome to the Spice AI take-home assessment! This assignment is designed for university candidates applying for full-time or intern positions. You'll build a simple Rust application leveraging Spice.ai Open Source Software (OSS). This is an opportunity to showcase your ability to learn and apply new concepts quickly.

**Estimated Time to Complete:** Less than 4 hours  
**Reward:** Submissions that meet all criteria will receive a Spice AI swag pack!

---

## Table of Contents

- [Spice AI Take-Home Assessment](#spice-ai-take-home-assessment)
  - [Table of Contents](#table-of-contents)
  - [Overview](#overview)
  - [Requirements](#requirements)
  - [Submission Guidelines](#submission-guidelines)
  - [Grading Criteria](#grading-criteria)
  - [Extra Credit Opportunities](#extra-credit-opportunities)
  - [Resources](#resources)
  - [Getting Help](#getting-help)

---

## Overview

Your task is to create a simple Rust application that utilizes Spice.ai OSS for data and AI-driven functionality. You'll use this GitHub template to start your project, and your final submission will be a GitHub repository based on this template containing all required components.

---

## Requirements

Your repository should include the following:

1. **Rust Application**

   - A simple application built in Rust that demonstrates your ability to use Rust effectively.
   - Incorporate Spice.ai OSS to enhance your application's functionality.

2. **One or More `spicepod.yml` Files**

   - Define your data connectors, embeddings, and models using `spicepod.yml`.
   - Ensure your Spicepods are well-structured and documented.

3. **Documentation**

   - Provide clear documentation explaining:
     - The purpose of your application.
     - How to build and run it.
     - Any dependencies or setup instructions.
     - An explanation of your Spicepods.
     - Any credentials required.

4. **GitHub Issues for Project Management**
   - Use GitHub Issues to track your project's tasks and progress.
   - Include meaningful commit messages linked to your issues.

---

## Submission Guidelines

1. **Use this GitHub Template**

   - Use this repository template to create your repository.
     
  <img width="433" alt="image" src="https://github.com/user-attachments/assets/08bec3eb-8afc-4951-ba5a-78b762a055b1">

2. **Develop Your Application**

   - Implement your Rust application and Spicepods within the repository.
   - Regularly commit your changes with descriptive messages.

3. **Manage Your Project with GitHub Issues**

   - Create issues for tasks, features, and bugs.
   - Close issues via commit messages (e.g., `Fixes #1 - Implement data connector`).

4. **Document Your Work**

   - Update the `README.md` in your repository with your application's documentation.

5. **Submit Your Repository**
   - Once complete, ensure your repository is public.
   - Email the link to your recruiter or submit it as per the application instructions.

---

## Grading Criteria

Your submission will be evaluated based on:

1. **Use of Data Connectors, Embeddings, and Models**

   - Diversity and creativity in using Spice.ai features.

2. **Code Quality**

   - Clean, efficient, and well-documented Rust code.
   - Proper use of SQL if applicable.

3. **Application Usefulness**
   - The practicality and originality of your application.

---

## Extra Credit Opportunities

Boost your submission by:

1. **Community Engagement**

   - Help others by answering questions in Discord.

2. **Submitting High-Quality Bug Reports**
   - Report any issues you find to [Spice.ai GitHub Issues](https://github.com/spiceai/spiceai/issues) with detailed information.

---

## Resources

- **Spice.ai OSS Rust SDK**

  - Office SDK for Rust: [github.com/spiceai/spice-rs](https://github.com/spiceai/spice-rs)

- **Spice.ai OSS Samples**

  - Explore example applications: [spiceai/samples](https://github.com/spiceai/samples)

- **Spice.ai OSS Quickstarts**

  - Get started quickly with these guides: [spiceai/quickstarts](https://github.com/spiceai/quickstarts)

- **Spice.ai OSS Documentation**

  - Official docs: [docs.spiceai.org](https://docs.spiceai.org)

- **Rust Language Resources**
  - Rust documentation: [doc.rust-lang.org](https://doc.rust-lang.org)

---

## Getting Help

- **Discord Community**

  - Join our Discord server and ask and answer questions in the `#help` channel: [Spice AI Discord](https://discord.gg/kZnTfneP5u)

---

**Good luck!**

## Table Definitions
```
CREATE TABLE wa_ev_population_data (
vin VARCHAR(10),
county VARCHAR(50),
city VARCHAR(50),
state CHAR(2),
postal_code VARCHAR(10),
model_year SMALLINT,
make VARCHAR(30),
model VARCHAR(30),
electric_vehicle_type VARCHAR(50),
cafv_eligibility VARCHAR(100),
electric_range SMALLINT,
base_msrp DECIMAL(10, 2),
legislative_district SMALLINT,
dol_vehicle_id BIGINT,
vehicle_location GEOGRAPHY,
electric_utility VARCHAR(100),
census_tract BIGINT
);
```
```
CREATE TABLE clean_alternative_fuel_vehicles (
clean_alt_fuel_vehicle_type VARCHAR(60),
vin VARCHAR(10),
dol_vehicle_id BIGINT,
model_year SMALLINT,
make VARCHAR(30),
model VARCHAR(30),
primary_use VARCHAR(20),
electric_range SMALLINT,
odometer_reading BIGINT,
odometer_reading_description VARCHAR(50),
new_or_used_vehicle VARCHAR(10),
sale_price DECIMAL(10, 2),
sale_date DATE,
base_msrp DECIMAL(10, 2),
transaction_type VARCHAR(50),
transaction_date DATE,
year SMALLINT,
county VARCHAR(50),
city VARCHAR(50),
state CHAR(2),
postal_code VARCHAR(10),
hb_2042_cafv_eligibility VARCHAR(100),
meets_2019_hb_2042_electric_range_req BOOLEAN,
meets_2019_hb_2042_sale_date_req BOOLEAN,
meets_2019_hb_2042_sale_price_value_req BOOLEAN,
2019_hb_2042_battery_range_req_met VARCHAR(100),
2019_hb_2042_purchase_date_requirement VARCHAR(100),
2019_hb_2042_sale_price_value_requirement VARCHAR(100),
electric_vehicle_fee_paid VARCHAR(20),
transportation_electrification_fee_paid VARCHAR(20),
hybrid_vehicle_electrification_fee_paid VARCHAR(20),
geoid BIGINT,
legislative_district SMALLINT,
electric_utility VARCHAR(100)
);
```
