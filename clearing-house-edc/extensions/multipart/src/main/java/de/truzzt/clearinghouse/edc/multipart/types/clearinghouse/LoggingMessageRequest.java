/*
 *  Copyright (c) 2021 Microsoft Corporation
 *
 *  This program and the accompanying materials are made available under the
 *  terms of the Apache License, Version 2.0 which is available at
 *  https://www.apache.org/licenses/LICENSE-2.0
 *
 *  SPDX-License-Identifier: Apache-2.0
 *
 *  Contributors:
 *       Microsoft Corporation - Initial implementation
 *
 */

package de.truzzt.clearinghouse.edc.multipart.types.clearinghouse;

import com.fasterxml.jackson.annotation.JsonProperty;
import org.jetbrains.annotations.NotNull;

public class LoggingMessageRequest {

    @JsonProperty("header")
    @NotNull
    private RequestHeader header;

    @JsonProperty("payload")
    @NotNull
    private String payload;

    public LoggingMessageRequest(@NotNull RequestHeader header, @NotNull String payload) {
        this.header = header;
        this.payload = payload;
    }
}

