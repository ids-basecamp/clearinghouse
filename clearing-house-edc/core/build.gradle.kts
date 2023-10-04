/*
 *  Copyright (c) 2023 Microsoft Corporation
 *
 *  This program and the accompanying materials are made available under the
 *  terms of the Apache License, Version 2.0 which is available at
 *  https://www.apache.org/licenses/LICENSE-2.0
 *
 *  SPDX-License-Identifier: Apache-2.0
 *
 *  Contributors:
 *       Microsoft Corporation - Initial implementation
 *       truzzt GmbH - EDC extension implementation
 *
 */

plugins {
    `java-library`
}

val auth0JWTVersion: String by project

dependencies {
    api(edc.spi.core)

    implementation(edc.ids)
    implementation(edc.ids.jsonld.serdes)
    implementation(edc.api.management.config)

    implementation("com.auth0:java-jwt:${auth0JWTVersion}")
}
